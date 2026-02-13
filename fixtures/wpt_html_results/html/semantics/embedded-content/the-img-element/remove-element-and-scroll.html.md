# html/semantics/embedded-content/the-img-element/remove-element-and-scroll.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/remove-element-and-scroll.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Images with loading='lazy' load being removed and then scrolled to</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="common.js"></script>
</head>

<body>
  <img id="in_viewport" src='resources/image.png?in_viewport&pipe=trickle(d1)'>
  <div style="height:1000vh"></div>
  <div id="below_viewport_div"></div>
  <img id="below_viewport" src='resources/image.png?below_viewport' loading="lazy">

  <script>
    const in_viewport_element = document.getElementById("in_viewport");
    const below_viewport_element = document.getElementById("below_viewport");
    const below_viewport_div = document.getElementById("below_viewport_div");

    async_test(t => {
      below_viewport_element.onload = t.unreached_func("Removed loading=lazy image " +
        "should not load when its old position is scrolled to.");
      below_viewport_element.remove();

      in_viewport_element.onload = () => {
        below_viewport_div.scrollIntoView();
        t.step_timeout(t.step_func_done(), 2000);
      };
    }, "Test that <img> below viewport is not loaded when removed from the " +
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
        "byte_end": 348,
        "byte_start": 271,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 506,
        "byte_start": 425,
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
  "source_name": "html/semantics/embedded-content/the-img-element/remove-element-and-scroll.html"
}
```
