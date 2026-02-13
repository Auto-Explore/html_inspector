# html/semantics/selectors/pseudo-classes/dir01.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/dir01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=iso-8859-8 id=meta>
<title id=title>Selector: pseudo-classes (:dir(ltr), :dir(rtl)) in iso-8859-8 documents</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<script src="/resources/testharness.js" id=script1></script>
<script src="/resources/testharnessreport.js" id=script2></script>
<script src="utils.js" id=script3></script>
<div id="log"></div>
<div>This text is left to right<div id=div1 style="direction:rtl">this is right to left</div></div>
<div>This text is left to right<span id=div2 style="direction:rtl">this is left to right</span></div>

<script>
  var ltr = new Array(),
      all = document.querySelectorAll('*');
  for(var i = all.length; i--; ltr.unshift(all[i]));
  testSelectorElementsMatch(":dir(ltr)", ltr, "direction doesn't affect :dir()");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “iso-8859-8” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 49,
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
  "source_name": "html/semantics/selectors/pseudo-classes/dir01.html"
}
```
