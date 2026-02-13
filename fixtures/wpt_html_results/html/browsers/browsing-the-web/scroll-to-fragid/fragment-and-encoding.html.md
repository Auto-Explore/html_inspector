# html/browsers/browsing-the-web/scroll-to-fragid/fragment-and-encoding.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/fragment-and-encoding.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=windows-1252>
<title>Fragment navigation: encoding</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div style=height:10000px></div>
<div id=&#xFF;></div>
<div id=&#xFEFF;></div>
<div id=&#x2661;&#x00FF;><div>
<script>
function goToTop() {
  location.hash = "top";
  assert_equals(self.scrollY, 0, "#top");
}

test(() => {
  assert_equals(location.hash, "", "Page must be loaded with no hash");

  location.hash = "\u00FF";
  assert_equals(location.hash, "#%C3%BF");
  assert_greater_than(self.scrollY, 1000, "#%C3%BF");
}, "U+00FF should find U+00FF");

test(() => {
  goToTop();

  location.hash = "%EF%BB%BF";
  assert_greater_than(self.scrollY, 1000, "#%EF%BB%BF");
}, "Percent-encoded UTF-8 BOM should find U+FEFF as BOM is not stripped when decoding");

test(() => {
  goToTop();

  location.hash = "%FF";
  assert_equals(self.scrollY, 0, "#%FF");
}, "%FF should not find U+00FF as decoding it gives U+FFFD");

test(() => {
  goToTop();

  // U+2661 in UTF-8 + %FF.
  // Chrome had an issue that the following fragment was decoded as U+2661 U+00FF.
  // https://github.com/whatwg/html/pull/3111
  location.hash = "%E2%99%A1%FF";
  assert_equals(self.scrollY, 0, "%E2%99%A1%FF");

  goToTop();
}, "Valid UTF-8 + invalid UTF-8 should not be matched to the utf8-decoded former + the isomorphic-decoded latter");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1252” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 43,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/fragment-and-encoding.html"
}
```
