# html/syntax/xmldecl/xmldecl-1.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/xmldecl/xmldecl-1.html",
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
<iframe src="support/baseline.htm"></iframe>
<iframe src="support/cp1251.htm"></iframe>
<iframe src="support/kilobyte-after.htm"></iframe>
<iframe src="support/kilobyte-before.htm"></iframe>
<iframe src="support/letter-between-xml-and-encoding.htm"></iframe>
<iframe src="support/lt-between-xml-and-encoding.htm"></iframe>
<iframe src="support/meta-inside-xml-charset-before-encoding.htm"></iframe>
<iframe src="support/meta-inside-xml-encoding-before-charset.htm"></iframe>
<iframe src="support/no-version.htm"></iframe>
<iframe src="support/no-version-or-space.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-body.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-body-single-quotes.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-body-single-quotes-spaces-and-line-breaks-around-equals.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-body-single-quotes-spaces-around-equals.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-lt.htm"></iframe>
<iframe src="support/one-around-equals.htm"></iframe>
<iframe src="support/VERSION.htm"></iframe>
<iframe src="support/WINDOWS.htm"></iframe>
<iframe src="support/zero-around-equals.htm"></iframe>
<iframe src="support/baseline-trail.htm"></iframe>
<iframe src="support/cp1251-trail.htm"></iframe>
<iframe src="support/kilobyte-after-trail.htm"></iframe>
<iframe src="support/kilobyte-before-trail.htm"></iframe>
<iframe src="support/letter-between-xml-and-encoding-trail.htm"></iframe>
<iframe src="support/lt-between-xml-and-encoding-trail.htm"></iframe>
<iframe src="support/meta-inside-xml-charset-before-encoding-trail.htm"></iframe>
<iframe src="support/meta-inside-xml-encoding-before-charset-trail.htm"></iframe>
<iframe src="support/no-version-trail.htm"></iframe>
<iframe src="support/no-version-or-space-trail.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trail.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-body-trail.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-body-single-quotes-trail.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-body-single-quotes-spaces-and-line-breaks-around-equals-trail.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-body-single-quotes-spaces-around-equals-trail.htm"></iframe>
<iframe src="support/no-version-or-space-or-trailing-question-trailing-lt-trail.htm"></iframe>
<iframe src="support/one-around-equals-trail.htm"></iframe>
<iframe src="support/VERSION-trail.htm"></iframe>
<iframe src="support/WINDOWS-trail.htm"></iframe>
<iframe src="support/zero-around-equals-trail.htm"></iframe>
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
        "byte_end": 3346,
        "byte_start": 3336,
        "col": 1,
        "line": 59
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
  "source_name": "html/syntax/xmldecl/xmldecl-1.html"
}
```
