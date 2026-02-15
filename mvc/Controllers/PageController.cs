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

        [HttpGet("{*slute}")]
        public ActionResult Index(string slute)
        {
            var page = _context.WikiArtikels.FirstOrDefault(w => w.Slug == slute);
            ViewBag.UrlSlug = slute;
            return View(page);
        }

    }
}
