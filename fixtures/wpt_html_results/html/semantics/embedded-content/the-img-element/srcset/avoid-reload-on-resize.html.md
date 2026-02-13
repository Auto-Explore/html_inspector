# html/semantics/embedded-content/the-img-element/srcset/avoid-reload-on-resize.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/srcset/avoid-reload-on-resize.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Avoid srcset image reloads when viewport resizes</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
setup({single_test:true});
const image_was_loaded = () => {
  const iframe = document.getElementById("iframe");
  // Resize the iframe
  iframe.width="400";
  // Wait 500 ms
  step_timeout(() => {
    // Check that the iframe only loaded a single resource
    const entries = iframe.contentWindow.performance.getEntriesByType("resource");
    assert_equals(entries.length, 1);
    done();
  }, 500);
}
</script>
<iframe id=iframe width="401" src="resources/resized.html" onload="image_was_loaded()"></iframe>
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
  "source_name": "html/semantics/embedded-content/the-img-element/srcset/avoid-reload-on-resize.html"
}
```
