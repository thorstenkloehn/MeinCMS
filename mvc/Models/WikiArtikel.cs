using System.Collections.Generic;

namespace mvc.Models;

public class WikiArtikel
{
    public long Id { get; set; }
    
    public string Slug { get; set; }
    public List<WikiArtikelVersion> Versionen { get; set; } = [];
   
}