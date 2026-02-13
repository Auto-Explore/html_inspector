# html/semantics/grouping-content/the-pre-element/pre-newline-bidi.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-pre-element/pre-newline-bidi.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8"/>
    <title>HTML Test: newline in pre separates bidi paragraphs</title>
    <link rel="match" href="pre-newline-bidi-ref.html" />
    <link rel="author" title="Amir E. Aharoni" href="mailto:amir.aharoni@mail.huji.ac.il"/>
    <link rel="author" title="Eyal Sela" href="mailto:eyal@post.isoc.org.il"/>
    <link rel="author" title="Aharon Lanin" href="mailto:aharon@google.com"/>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-pre-element"/>
    <meta name="assert"
      content="A newline in a pre element should separate paragraphs for the purposes of the Unicode bidirectional algorithm."/>
  </head>
  <body>
    <div class="instructions"><p>Test passes if the rightmost character in the first line below is a full stop and to the left of it is a Hebrew letter.</p></div>
    <div class="test">
      <pre>
A Hebrew letter and a full stop: &#x05d0;.
&#x05d0; this line begins with a Hebrew letter.
      </pre>
    </div>
  </body>
</html>
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
  "source_name": "html/semantics/grouping-content/the-pre-element/pre-newline-bidi.html"
}
```
