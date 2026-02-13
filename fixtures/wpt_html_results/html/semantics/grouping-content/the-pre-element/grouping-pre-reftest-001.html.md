# html/semantics/grouping-content/the-pre-element/grouping-pre-reftest-001.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-pre-element/grouping-pre-reftest-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>pre element</title>
    <link rel="author" title="dzenana" href="mailto:dzenana.trenutak@gmail.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-pre-element">
    <link rel="match" href="grouping-pre-reftest-001-ref.html" />
    <meta name="assert" content="Newlines within pre elements separate paragraphs for the purposes of BIDI." />
</head>
<body>
    <h1>Description</h1>
    <p>This test continues to validate the pre element.</p>

    <p>The spec states:</p>
    <blockquote>"A newline in a pre element should separate paragraphs for the purposes of the Unicode bidirectional algorithm. This requirement may be implemented indirectly through the style layer. For example, an HTML+CSS user agent could implement these requirements by implementing the CSS 'unicode-bidi' property."</blockquote>

    <p>This reftest passes if below you see "ABC ABC" repeated on two separate lines below (4 ABCs total):</p>

    <pre>&#x202E CBA CBA
ABC ABC</pre>

</html>
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
        "byte_end": 1018,
        "byte_start": 1017,
        "col": 10,
        "line": 20
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
  "source_name": "html/semantics/grouping-content/the-pre-element/grouping-pre-reftest-001.html"
}
```
