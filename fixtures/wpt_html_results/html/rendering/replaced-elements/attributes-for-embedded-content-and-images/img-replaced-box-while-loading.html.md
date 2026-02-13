# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-replaced-box-while-loading.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-replaced-box-while-loading.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Images don't render as a non-replaced inline while loading, even when there's no concrete size specified</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1472637">
<style>
  img {
    min-width: 1000px;
  }
</style>
<img alt="T">
<script>
// Do an async test off the onload handler to avoid waiting for the load even for too long unnecessarily.
let t = async_test("Loading images should get a replaced box, even without specified size");
onload = t.step_func(function() {
  const image = document.querySelector("img");
  // Use the trickle pipe to have 100 seconds until the image actually loads,
  // that should be enough to run the test.
  image.src = "../../../../../images/blue.png?pipe=trickle(d100)";
  t.step_timeout(t.step_func_done(function() {
    assert_equals(
      image.offsetWidth,
      1000,
    );
  }), 0);
});
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
        "byte_end": 529,
        "byte_start": 516,
        "col": 1,
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/img-replaced-box-while-loading.html"
}
```
