# html/semantics/document-metadata/the-meta-element/http-equiv-and-name-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/http-equiv-and-name-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Setting both http-equiv and name attributes on a meta element</title>
<meta http-equiv=content-language name=color-scheme content=dark>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="color-scheme/support/compute-root-color-scheme.js"></script>
<!--
  NOTE: This test assumes that the browser's default color-scheme is "light",
  see https://github.com/web-platform-tests/wpt/pull/31268 for reasoning
-->
<script>
  // This creates a test()
  assert_root_color_scheme("dark", "<meta> set the color-scheme to dark");

  // We can't test content-language against :lang(), because CSS Selectors 4
  // references BCP 47 syntax and RFC4647 "Matching of Language Tags", but
  // "dark" is not a well-formed BCP 47 tag and therefore cannot be matched.
  // Therefore, the test that content-language gets set is split off to a
  // separate testcase using a well-formed lang tag as the content.
  // test(() => {
  //   assert_equals(document.querySelector(":root:lang(dark)"), document.documentElement);
  // }, "<meta> set the content-language to dark");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.http_equiv.content_language.obsolete",
      "message": "Using the “meta” element to specify the document-wide default language is obsolete. Consider specifying the language on the root element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 158,
        "byte_start": 93,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/document-metadata/the-meta-element/http-equiv-and-name-1.html"
}
```
