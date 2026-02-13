# html/semantics/document-metadata/the-link-element/link-rellist.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-rellist.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>link.relList: non-string contains</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-link-element">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#domtokenlist">
<link rel="help" href="https://webidl.spec.whatwg.org/#ecmascript-binding">
<link rel="help" href="http://www.ecma-international.org/publications/files/ECMA-ST/ECMA-262.pdf#page=57">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link id="link" rel="undefined null 0 NaN Infinity">
<div id="log"></div>
<script>
test(function() {
  var list = document.getElementById("link").relList;
  assert_equals(list.contains(undefined), true); //"undefined"
  assert_equals(list.contains(null), true); //"null"
  assert_equals(list.contains(-0), true); //"0"
  assert_equals(list.contains(+0), true); //"0"
  assert_equals(list.contains(NaN), true); //"NaN"
  assert_equals(list.contains(+Infinity), true); //"Infinity"
  assert_equals(list.contains(-Infinity), false); //"-Infinity"
  assert_equals(list.supports("stylesheet"), true);
  assert_equals(list.supports("nosuchrelvalueever"), false);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 632,
        "byte_start": 580,
        "col": 1,
        "line": 10
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-rellist.html"
}
```
