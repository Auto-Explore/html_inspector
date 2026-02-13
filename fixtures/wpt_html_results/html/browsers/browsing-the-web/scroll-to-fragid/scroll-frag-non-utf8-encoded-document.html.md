# html/browsers/browsing-the-web/scroll-to-fragid/scroll-frag-non-utf8-encoded-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/scroll-frag-non-utf8-encoded-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Fragment Navigation: fragment id should not be found in non UTF8 document</title>
<meta name=timeout content=long>
<meta http-equiv="Content-Type" content="text/html; charset=gbk"/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<div></div>
<div id="&#x586f" style="position:absolute; top:100px;"></div>
<div style="height:200vh;"></div>
<script>
async_test(test => {
  assert_equals(document.characterSet, "GBK", "Document should be GBK encoded");
  assert_equals(location.hash, "", "Page must be loaded with no hash");
  location.hash = '%89g';
  test.step_timeout(() => {
    assert_equals( document.scrollingElement.scrollTop, 0 );
    test.done();
  }, 1);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.charref_no_semicolon",
      "message": "Character reference was not terminated by a semicolon.",
      "severity": "Warning",
      "span": {
        "byte_end": 331,
        "byte_start": 330,
        "col": 1,
        "line": 9
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
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/scroll-frag-non-utf8-encoded-document.html"
}
```
