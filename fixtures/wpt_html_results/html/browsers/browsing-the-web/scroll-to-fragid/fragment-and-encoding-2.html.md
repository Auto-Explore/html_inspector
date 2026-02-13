# html/browsers/browsing-the-web/scroll-to-fragid/fragment-and-encoding-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/fragment-and-encoding-2.html",
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
<div id=&#xFFFD;></div>
<div id=&#xFEFF;&#xFFFD;></div>
<script>
function goToTop() {
  location.hash = "top";
  assert_equals(self.scrollY, 0, "#top");
}

test(() => {
  assert_equals(location.hash, "", "Page must be loaded with no hash");

  location.hash = "%C2";
  assert_equals(location.hash, "#%C2");
  assert_greater_than(self.scrollY, 1000, "#%C2");
}, "Invalid percent-encoded UTF-8 byte should decode as U+FFFD");

test(() => {
  goToTop();

  location.hash = "%EF%BB%BF%C2";
  assert_equals(location.hash, "#%EF%BB%BF%C2");
  assert_greater_than(self.scrollY, 1000, "#%EF%BB%BF%C2");
}, "Percent-encoded UTF-8 BOM followed by invalid UTF-8 byte should decode as U+FEFF U+FFFD");

test(() => {
  goToTop();

  location.hash = "%EF%BF%BD";
  assert_equals(location.hash, "#%EF%BF%BD");
  assert_greater_than(self.scrollY, 1000, "#%EF%BF%BD");

  goToTop();
}, "Percent-encoded UTF-8 byte sequence for U+FFFD should decode as U+FFFD");
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
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/fragment-and-encoding-2.html"
}
```
