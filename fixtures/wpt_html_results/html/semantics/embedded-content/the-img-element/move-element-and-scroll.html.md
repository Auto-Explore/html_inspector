# html/semantics/embedded-content/the-img-element/move-element-and-scroll.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/move-element-and-scroll.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Images with loading='lazy' load being moved to another document
         and then scrolled to</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="common.js"></script>
</head>

<body>
  <div id="tall_div" style="height:1000vh"></div>
  <div id="below_viewport_div"></div>
  <img id="below_viewport" src='resources/image.png?below_viewport' loading="lazy">

  <script>
    const tall_div = document.getElementById("tall_div");
    const below_viewport_element = document.getElementById("below_viewport");
    const below_viewport_div = document.getElementById("below_viewport_div");

    async_test(function(t) {
      below_viewport_element.onload =
        t.unreached_func("The below viewport image should not load");
      t.step_timeout(t.step_func_done(), 1000);
      const iframe = document.createElement('iframe');
      iframe.setAttribute("style", "display:none");
      iframe.srcdoc = "<body></body>";
      iframe.onload = () => {
        const adopted_img = iframe.contentDocument.adoptNode(below_viewport_element);
        iframe.contentDocument.body.appendChild(adopted_img);
        below_viewport_div.scrollIntoView();
      };
      document.body.insertBefore(iframe, tall_div);
    }, "Test that <img> below viewport is not loaded when moved to another " +
       "document and then scrolled to");
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 467,
        "byte_start": 386,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-img-element/move-element-and-scroll.html"
}
```
