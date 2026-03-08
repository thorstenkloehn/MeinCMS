using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using mvc.Data;
using mvc.Models;
using System.Text.RegularExpressions;
using Markdig;

namespace mvc.Controllers
{
    /// <summary>
    /// Controller zur Verwaltung und Anzeige von Wiki-Seiten.
    /// </summary>
    public class PageController : Controller
    {
        private readonly ApplicationDbContext _context;
        private readonly MarkdownPipeline _pipeline;

        /// <summary>
        /// Initialisiert eine neue Instanz des <see cref="PageController"/>.
        /// </summary>
        /// <param name="context">Der Datenbankkontext.</param>
        public PageController(ApplicationDbContext context)
        {
            _context = context;
            _pipeline = new MarkdownPipelineBuilder()
                .UseAdvancedExtensions()
                .Build();
        }

        /// <summary>
        /// Zeigt das Formular zum Erstellen einer neuen Wiki-Seite an.
        /// </summary>
        /// <param name="slug">Der gewünschte URL-Slug der neuen Seite.</param>
        /// <returns>Das View-Ergebnis für das Formular.</returns>
        [HttpGet("Neuformular/{*slug}/")]
        public ActionResult Neuformular(string slug)
        {
            if (!string.IsNullOrEmpty(slug) && !Regex.IsMatch(slug, @"^[a-zA-Z0-9/_-]+$"))
                return BadRequest("Ungültiger Slug.");

            ViewBag.UrlSlug = slug;
            return View();
        }

        /// <summary>
        /// Erstellt eine neue Version eines Wiki-Artikels. Falls der Artikel noch nicht existiert, wird er angelegt.
        /// </summary>
        /// <param name="slug">Der Slug des Artikels.</param>
        /// <param name="markdownInhalt">Der Inhalt im Markdown-Format.</param>
        /// <returns>Eine Weiterleitung zur Index-Ansicht des Artikels.</returns>
        [HttpPost("Create")]
        [ValidateAntiForgeryToken]
        public async Task<IActionResult> Create(string slug, string markdownInhalt)
        {
            if (string.IsNullOrEmpty(slug) || !Regex.IsMatch(slug, @"^[a-zA-Z0-9/_-]+$"))
                return BadRequest("Ungültiger Slug.");

            if (string.IsNullOrWhiteSpace(markdownInhalt))
                return BadRequest("Inhalt darf nicht leer sein.");

            var htmlInhalt = Markdown.ToHtml(markdownInhalt, _pipeline);

            var artikel = _context.WikiArtikels.FirstOrDefault(a => a.Slug == slug);
            if (artikel == null)
            {
                artikel = new WikiArtikel { Slug = slug };
                _context.WikiArtikels.Add(artikel);
            }

            var version = new WikiArtikelVersion
            {
                MarkdownInhalt = markdownInhalt,
                HtmlInhalt = htmlInhalt,
                Zeitpunkt = DateTime.UtcNow
            };

            artikel.Versionen.Add(version);
            await _context.SaveChangesAsync();

            return RedirectToAction("Index", new { slug = slug });
        }

        /// <summary>
        /// Zeigt das Bearbeitungsformular für eine bestehende Wiki-Seite an.
        /// </summary>
        /// <param name="slug">Der Slug der Seite.</param>
        /// <returns>Die Bearbeitungsansicht.</returns>
        [HttpGet("Edit/{*slug}")]
        public ActionResult Edit(string slug)
        {
            if (string.IsNullOrEmpty(slug) || !Regex.IsMatch(slug, @"^[a-zA-Z0-9/_-]+$"))
                return BadRequest("Ungültiger Slug.");

            var page = _context.WikiArtikels
                .Include(a => a.Versionen)
                .FirstOrDefault(w => w.Slug == slug);

            if (page == null)
                return NotFound();

            ViewBag.UrlSlug = slug;
            return View(page);
        }

        /// <summary>
        /// Verarbeitet die Bearbeitung einer Wiki-Seite.
        /// </summary>
        /// <param name="slug">Der Slug der Seite.</param>
        /// <param name="markdownInhalt">Der neue Inhalt.</param>
        /// <returns>Eine Weiterleitung zur Index-Ansicht.</returns>
        [HttpPost("Edit/{*slug}")]
        [ValidateAntiForgeryToken]
        public async Task<IActionResult> Edit(string slug, string markdownInhalt)
        {
            return await Create(slug, markdownInhalt);
        }

        /// <summary>
        /// Zeigt eine Wiki-Seite an. Wenn kein Slug angegeben ist, wird die "Hauptseite" geladen.
        /// </summary>
        /// <param name="slug">Der Slug der anzuzeigenden Seite.</param>
        /// <returns>Die Seite oder eine Fehlermeldung.</returns>
        [HttpGet("{*slug}")]
        public ActionResult Index(string slug)
        {
            if (string.IsNullOrEmpty(slug))
            {
                slug = "Hauptseite";
            }

            if (!Regex.IsMatch(slug, @"^[a-zA-Z0-9/_-]+$"))
                return BadRequest("Ungültiger Slug.");

            var page = _context.WikiArtikels
                .Include(a => a.Versionen)
                .FirstOrDefault(w => w.Slug == slug);
            
            ViewBag.UrlSlug = slug;
            return View(page);
        }
    }
}
