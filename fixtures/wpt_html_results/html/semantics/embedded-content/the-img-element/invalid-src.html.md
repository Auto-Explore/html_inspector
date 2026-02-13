# html/semantics/embedded-content/the-img-element/invalid-src.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/invalid-src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Loading a non-parsing URL as an image should silently fail; triggering appropriate events</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<img id=brokenurl />
<img id=emptysrc />
<script>
async_test(function(t) {
    var img = document.getElementById("brokenurl");
    img.src = "http://[";

    // The errors should be queued in the event loop, so they should only trigger
    // after this block of code finishes, not during the img.src setter itself
    img.addEventListener('error', t.step_func(function() {
        t.step_timeout(t.step_func_done(), 0);
    }));
}, 'src="http://["');

async_test(function(t) {
    var img = document.getElementById("emptysrc");
    img.src = "";

    // Setting src to empty string triggers only error event.
    // The errors should be queued in the event loop, so they should only trigger
    // after this block of code finishes, not during the img.src setter itself
    img.addEventListener('error', t.step_func(function() {
        t.step_timeout(t.step_func_done(), 0);
    }));
}, 'src=""');

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 270,
        "byte_start": 250,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 270,
        "byte_start": 250,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 290,
        "byte_start": 271,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 290,
        "byte_start": 271,
        "col": 1,
        "line": 7
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
  "source_name": "html/semantics/embedded-content/the-img-element/invalid-src.html"
}
```
