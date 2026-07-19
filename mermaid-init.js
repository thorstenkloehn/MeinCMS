document.addEventListener("DOMContentLoaded", function () {
  // Convert markdown code blocks with class language-mermaid to div.mermaid
  var mermaidBlocks = document.querySelectorAll("code.language-mermaid, pre code.language-mermaid");
  mermaidBlocks.forEach(function (block) {
    var pre = block.parentElement;
    var div = document.createElement("div");
    div.className = "mermaid";
    div.textContent = block.textContent;
    if (pre && pre.tagName === "PRE") {
      pre.parentNode.replaceChild(div, pre);
    } else {
      block.parentNode.replaceChild(div, block);
    }
  });

  if (typeof mermaid !== "undefined") {
    mermaid.initialize({
      startOnLoad: true,
      theme: "default",
      securityLevel: "loose"
    });
    mermaid.run();
  }
});
