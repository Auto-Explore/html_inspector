# html/links/manifest/link-relationship/link-rel-manifest.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/manifest/link-relationship/link-rel-manifest.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>
  Test that "manifest" is a supported value for the `rel` of a `link`
</title>
<link rel="help" href="https://html.spec.whatwg.org/#link-type-manifest" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  test(() => {
    const result = document.createElement("link").relList.supports("manifest");
    assert_true(
      result,
      "Expected true if manifest is supported as a link relationship"
    );
  }, 'link element supports a rel value of "manifest".');
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
  "source_name": "html/links/manifest/link-relationship/link-rel-manifest.html"
}
```
