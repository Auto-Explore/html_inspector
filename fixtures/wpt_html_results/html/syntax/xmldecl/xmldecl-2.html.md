# html/syntax/xmldecl/xmldecl-2.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/xmldecl/xmldecl-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="windows-1252">
<title>Bogo-XML declaration</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=support/test_support.js></script>

<script>
setup({explicit_done:true});
window.onload = function() {
  runAllTests();
  done();
};
</script>

<section style="display: none;">
<div class="windows-1251">
<iframe src="support/kilobyte-plus-one-after.htm"></iframe>
<iframe src="support/kilobyte-plus-one-before.htm"></iframe>
<iframe src="support/kilobyte-plus-one-after-trail.htm"></iframe>
<iframe src="support/kilobyte-plus-one-before-trail.htm"></iframe>
</div>
<div class="windows-1252">
<iframe src="support/encodingencoding.htm"></iframe>
<iframe src="support/encoding-equals-encoding.htm"></iframe>
<iframe src="support/ENCODING.htm"></iframe>
<iframe src="support/gt-between-xml-and-encoding.htm"></iframe>
<iframe src="support/no-quotes.htm"></iframe>
<iframe src="support/no-quotes-space.htm"></iframe>
<iframe src="support/one-around-label.htm"></iframe>
<iframe src="support/space-around-label.htm"></iframe>
<iframe src="support/space-before.htm"></iframe>
<iframe src="support/unmatched-quotes.htm"></iframe>
<iframe src="support/XML.htm"></iframe>
<iframe src="support/zero-around-label.htm"></iframe>
<iframe src="support/encodingencoding-trail.htm"></iframe>
<iframe src="support/encoding-equals-encoding-trail.htm"></iframe>
<iframe src="support/ENCODING-trail.htm"></iframe>
<iframe src="support/gt-between-xml-and-encoding-trail.htm"></iframe>
<iframe src="support/no-quotes-trail.htm"></iframe>
<iframe src="support/no-quotes-space-trail.htm"></iframe>
<iframe src="support/one-around-label-trail.htm"></iframe>
<iframe src="support/space-around-label-trail.htm"></iframe>
<iframe src="support/space-before-trail.htm"></iframe>
<iframe src="support/unmatched-quotes-trail.htm"></iframe>
<iframe src="support/XML-trail.htm"></iframe>
<iframe src="support/zero-around-label-trail.htm"></iframe>
</div>
<div class="windows-1253">
<iframe src="support/xml-and-meta.htm"></iframe>
<iframe src="support/incomplete-utf-16le-and-meta.htm"></iframe>
<iframe src="support/incomplete-utf-16be-and-meta.htm"></iframe>
<iframe src="support/xml-and-meta-trail.htm"></iframe>
<iframe src="support/incomplete-utf-16le-and-meta-trail.htm"></iframe>
<iframe src="support/incomplete-utf-16be-and-meta-trail.htm"></iframe>
</div>
<div class="UTF-16LE">
<iframe src="support/utf-16le-and-meta.htm"></iframe>
<iframe src="support/utf-16le-and-meta-trail.htm"></iframe>
</div>
<div class="UTF-16BE">
<iframe src="support/utf-16be-and-meta.htm"></iframe>
<iframe src="support/utf-16be-and-meta-trail.htm"></iframe>
</div>
<div class="UTF-8">
<iframe src="support/utf-16le-vs-http.html"></iframe>
<iframe src="support/utf-16be-vs-http.html"></iframe>
<iframe src="support/xml-vs-http.html"></iframe>
<iframe src="support/utf-16le-vs-http-trail.html"></iframe>
<iframe src="support/utf-16be-vs-http-trail.html"></iframe>
<iframe src="support/xml-vs-http-trail.html"></iframe>
<iframe src="support/ascii-decl-for-utf-16.htm"></iframe>
</div>
<div class="replacement">
<iframe src="support/replacement.htm"></iframe>
<iframe src="support/replacement-trail.htm"></iframe>
</div>
</section>
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
        "byte_end": 45,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 3276,
        "byte_start": 3266,
        "col": 1,
        "line": 78
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
  "source_name": "html/syntax/xmldecl/xmldecl-2.html"
}
```
