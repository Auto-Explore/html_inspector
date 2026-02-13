# html/semantics/embedded-content/media-elements/volume_nonfinite.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/volume_nonfinite.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Setting HTMLMediaElement.volume to non-finite numbers</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
["audio", "video"].forEach(function(aElement) {
  [NaN, Infinity, -Infinity].forEach(function(aValue) {
    test(function() {
      var el = document.createElement(aElement);
      assert_throws_js(TypeError, function() {
        el.volume = aValue;
      });
    }, "Setting " + aElement + ".volume to " + String(aValue) + " should throw a TypeError");
  });
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/media-elements/volume_nonfinite.html"
}
```
