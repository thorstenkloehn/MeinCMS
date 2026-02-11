using System;

namespace mvc.Models;

public class WikiArtikelVersion
{
    public long VersionNummer{get;set;}
     public string MarkdownInhalt { get; set; }
    public string HtmlInhalt { get; set; }
    public DateTime Zeitpunkt { get; set; }
    public List<string> Kategorie { get; set; } = [];

}
