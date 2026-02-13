# html/semantics/embedded-content/the-img-element/not-rendered-image-loading-lazy.html

Counts:
- errors: 3
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/not-rendered-image-loading-lazy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>In-viewport loading=lazy not-rendered images should never load</title>
  <link rel="author" title="Rob Buis" href="mailto:rbuis@igalia.com">
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/common.js"></script>
</head>

<body>
  <!-- These images must not attempt to load -->
  <img id="display_none" style="display:none;" src="resources/image.png?not-rendered-image-loading-lazy-2" loading="lazy"
       onload="display_none_img.resolve();" onerror="display_none_img.reject();">
  <img id="attribute_hidden" hidden src="resources/image.png?not-rendered-image-loading-lazy-3" loading="lazy"
       onload="attribute_hidden_img.resolve();" onerror="attribute_hidden_img.reject();">
  <img id="js_display_none" src="resources/image.png?not-rendered-image-loading-lazy-4" loading="lazy"
       onload="js_display_none_img.resolve();" onerror="js_display_none_img.reject();">
  <script>
    document.getElementById("js_display_none").style = 'display:none;';
  </script>
</body>

<script>
  const display_none_img = new ElementLoadPromise("display_none");
  const attribute_hidden_img = new ElementLoadPromise("attribute_hidden");
  const js_display_none_img = new ElementLoadPromise("js_display_none");

  async_test(t => {
    const unreached_not_rendered_img_func =
      t.unreached_func("The not-rendered in-viewport loading=lazy images " +
                       "should not attempt to load.");

    display_none_img.promise
      .then(unreached_not_rendered_img_func)
      .catch(unreached_not_rendered_img_func);

    attribute_hidden_img.promise
      .then(unreached_not_rendered_img_func)
      .catch(unreached_not_rendered_img_func);

    js_display_none_img.promise
      .then(unreached_not_rendered_img_func)
      .catch(unreached_not_rendered_img_func);

    t.step_timeout(t.done, 2000);
  }, "In-viewport loading=lazy not-rendered images should never load");
</script>

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
        "byte_end": 674,
        "byte_start": 473,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 875,
        "byte_start": 677,
        "col": 3,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1066,
        "byte_start": 878,
        "col": 3,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 1179,
        "byte_start": 1171,
        "col": 1,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 2072,
        "byte_start": 1179,
        "col": 9,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 2081,
        "byte_start": 2072,
        "col": 1,
        "line": 48
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
  "source_name": "html/semantics/embedded-content/the-img-element/not-rendered-image-loading-lazy.html"
}
```
