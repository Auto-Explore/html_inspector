# html/semantics/embedded-content/the-img-element/update-src-complete.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-src-complete.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Changing the img src should retain the 'complete' property</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<p id="display"><img src="image.png"></p>
<script>
    setup({ single_test: true });

    function check() {
        var img = document.querySelector("img");
        assert_true(img.complete, "By onload, image should have loaded");
        img.src = `image.png?${Math.random()}`;
        assert_false(img.complete, "Now that we're loading we should no longer be complete");
        img.onload = function () {
            assert_true(img.complete, "The new thing should have loaded.");
            done();
        }
    }

    onload = function () {
        check();
    };

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
        "byte_end": 256,
        "byte_start": 235,
        "col": 17,
        "line": 6
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
  "source_name": "html/semantics/embedded-content/the-img-element/update-src-complete.html"
}
```
