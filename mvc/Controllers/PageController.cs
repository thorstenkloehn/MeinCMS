using Microsoft.AspNetCore.Mvc;

namespace mvc.Controllers
{
    public class PageController : Controller
    {
        // GET: PageController
      public ActionResult Index(string slute)
{
    ViewBag.slute = slute;
    return View();
}


    }
}
