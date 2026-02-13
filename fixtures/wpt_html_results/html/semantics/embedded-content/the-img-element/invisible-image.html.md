# html/semantics/embedded-content/the-img-element/invisible-image.html

Counts:
- errors: 0
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/invisible-image.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>Test that below-viewport invisible images that are not marked
         loading=lazy still load, and block the window load event</title>
  <link rel="author" title="Rob Buis" href="mailto:rbuis@igalia.com">
  <link rel="author" title="Dom Farolino" href="mailto:dom@chromium.org">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="common.js"></script>
</head>

<body>
  <div style="height:1000vh;"></div>
  <img id="visibility_hidden" style="visibility:hidden;" src='resources/image.png?1'>
  <img id="visibility_hidden_explicit_eager" style="visibility:hidden;" src='resources/image.png?2'
       loading="eager">

  <img id="display_none" style="display:none;" src='resources/image.png?3'>
  <img id="display_none_explicit_eager" style="display:none;" src='resources/image.png?4'
       loading="eager">

  <img id="attribute_hidden" hidden src='resources/image.png?5'>
  <img id="attribute_hidden_explicit_eager" hidden src='resources/image.png?6'
       loading="eager">

  <img id="js_display_none" src='resources/image.png?7'>
  <img id="js_display_none_explicit_eager" src='resources/image.png?8'
       loading="eager">
  <script>
    document.getElementById("js_display_none").style = 'display:none;';

    const visibility_hidden_element = document.getElementById("visibility_hidden");
    const visibility_hidden_element_explicit_eager =
      document.getElementById("visibility_hidden_explicit_eager");

    const display_none_element = document.getElementById("display_none");
    const display_none_element_explicit_eager =
      document.getElementById("display_none_explicit_eager");

    const attribute_hidden_element = document.getElementById("attribute_hidden");
    const attribute_hidden_element_explicit_eager =
      document.getElementById("attribute_hidden_explicit_eager");

    const js_display_none_element = document.getElementById("js_display_none");
    const js_display_none_element_explicit_eager =
      document.getElementById("js_display_none_explicit_eager");

    let have_images_loaded = false;

    async_test(t => {
      let image_fully_loaded_promise = (element) => {
        return new Promise(resolve => {
          element.addEventListener("load", t.step_func(resolve));
        });
      }

      Promise.all([
        image_fully_loaded_promise(visibility_hidden_element),
        image_fully_loaded_promise(visibility_hidden_element_explicit_eager),
        image_fully_loaded_promise(display_none_element),
        image_fully_loaded_promise(display_none_element_explicit_eager),
        image_fully_loaded_promise(attribute_hidden_element),
        image_fully_loaded_promise(attribute_hidden_element_explicit_eager),
        image_fully_loaded_promise(js_display_none_element),
        image_fully_loaded_promise(js_display_none_element_explicit_eager)
      ]).then(t.step_func(() => {
        have_images_loaded = true;
      })).catch(t.unreached_func("All images should load correctly"));

      window.addEventListener("load", t.step_func_done(() => {
        assert_true(have_images_loaded,
                    "The images should block the window load event.");
      }));

    }, "Test that below-viewport invisible images that are not marked " +
       "loading=lazy still load, and block the window load event");
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
        "byte_end": 596,
        "byte_start": 513,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 720,
        "byte_start": 599,
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
        "byte_end": 797,
        "byte_start": 724,
        "col": 3,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 911,
        "byte_start": 800,
        "col": 3,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 977,
        "byte_start": 915,
        "col": 3,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1080,
        "byte_start": 980,
        "col": 3,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1138,
        "byte_start": 1084,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1233,
        "byte_start": 1141,
        "col": 3,
        "line": 27
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
  "source_name": "html/semantics/embedded-content/the-img-element/invisible-image.html"
}
```
