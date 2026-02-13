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
 public ActionResult Index(string slute)
{
    var page = _context.WikiArtikels.FirstOrDefault(w => w.Slug == slute);
   
  
    return View(page);
}


    }
}
