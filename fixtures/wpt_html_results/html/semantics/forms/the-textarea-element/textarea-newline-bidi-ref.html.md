# html/semantics/forms/the-textarea-element/textarea-newline-bidi-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-newline-bidi-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>HTML Test reference: newline in &lt;textarea&gt; separates bidi paragraphs</title>
    <link rel="author" title="Amir E. Aharoni" href="mailto:amir.aharoni@mail.huji.ac.il"/>
    <link rel="author" title="Eyal Sela" href="mailto:eyal@post.isoc.org.il"/>
    <link rel="author" title="Aharon Lanin" href="mailto:aharon@google.com"/>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-textarea-element"/>
  </head>
  <body>
    <div class="instructions"><p>Test passes if the rightmost character in the first line below is a full stop and to the left of it is a Hebrew letter.</p></div>
    <div class="test">
      <textarea cols="70" rows="3">
A Hebrew letter and a full stop: &#x05d0;.&lrm;
&#x05d0; this line begins with a Hebrew letter.
      </textarea>
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-newline-bidi-ref.html"
}
```
