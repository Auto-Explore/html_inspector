# html/dom/documents/dom-tree-accessors/document.title-05.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.title-05.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.title and White_Space characters</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#document.title">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var White_Space = [
  "\u000B",
  "\u0085",
  "\u00A0",
  "\u1680",
  "\u180E",
  "\u2000",
  "\u2001",
  "\u2002",
  "\u2003",
  "\u2004",
  "\u2005",
  "\u2006",
  "\u2007",
  "\u2008",
  "\u2009",
  "\u200A",
  "\u2028",
  "\u2029",
  "\u202F",
  "\u205F",
  "\u3000"
];

White_Space.forEach(function(character, i) {
  test(function() {
    var s = character + "a" + character + character + "b" + character + "c" +
      String(i) + character;
    document.title = s;
    assert_equals(document.title, s);
  }, "Removing whitespace in document.title: U+" +
     character.charCodeAt(0).toString(16));
});
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.title-05.html"
}
```
