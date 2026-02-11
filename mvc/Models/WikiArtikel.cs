using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;

namespace mvc.Models;

public class WikiArtikel
{
    [Key]
    public long Id { get; set; }
    
    public string Slug { get; set; }
    public List<WikiArtikelVersion> Versionen { get; set; } = [];
   
}