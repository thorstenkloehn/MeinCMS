using Microsoft.AspNetCore.Mvc;
using mvc.Data;
using mvc.Migrations;

namespace mvc.Controllers
{
    public class PageController : Controller
    {
        // GET: PageController

        private readonly ApplicationDbContext _context;
        public PageController(ApplicationDbContext context)
        {
            _context = context;
        }
        [HttpGet("Neuformular/{*slute}")]
        public ActionResult Neuformular(string slute) {
               ViewBag.UrlSlug = slute;
        return View();
        }

 

      
        public ActionResult Edit(string slute)
        {
            var page = _context.WikiArtikels.FirstOrDefault(w => w.Slug == slute);
            if (page == null)
                return NotFound();
            ViewBag.UrlSlug = slute;
            return View(page);
        }

 [HttpGet("{*slug}")]
public ActionResult Index(string slug)
{
   if (string.IsNullOrEmpty(slug))
{
    slug = "Hauptseite";
}
    // Regex-Prüfung im Code
    if (!System.Text.RegularExpressions.Regex.IsMatch(slug, @"^[a-zA-Z0-9/_-]+$"))
        return BadRequest("Ungültiger Url.");
    var page = _context.WikiArtikels.FirstOrDefault(w => w.Slug == slug);
    ViewBag.UrlSlug = slug;
    return View(page);
}

    }
}
