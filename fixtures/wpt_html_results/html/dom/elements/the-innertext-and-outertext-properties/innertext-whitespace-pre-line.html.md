# html/dom/elements/the-innertext-and-outertext-properties/innertext-whitespace-pre-line.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/the-innertext-and-outertext-properties/innertext-whitespace-pre-line.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>innerText with white-space:pre-line</title>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1923829">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="a" style="white-space: pre-line">one&#10;two&#10;three&#10;four</div>

<div id="b" style="white-space: pre">one&#10;two&#10;three&#10;four</div>

<div id="c" style="white-space: pre-line">
 one
  two
    <!-- comment -->
   three
    four
</div>

<div id="d" style="white-space: pre">
 one
  two
    <!-- comment -->
   three
    four
</div>

<script>
test(() => {
  assert_equals(a.innerText, b.innerText);
}, "innerText should be the same for the pre-line and pre examples");

test(() => {
  function collapseWhitespace(s) {
    return s.replace(/  +/g, ' ')  // collapse runs of spaces
            .replace(/ $/mg, '')   // strip trailing spaces
            .replace(/^ /mg, '');  // strip leading spaces
  }
  assert_equals(c.innerText, collapseWhitespace(d.innerText));
}, "innerText has collapsed whitespace but preserved newlines with pre-line");
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
  "source_name": "html/dom/elements/the-innertext-and-outertext-properties/innertext-whitespace-pre-line.html"
}
```
