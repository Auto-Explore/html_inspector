# html/semantics/forms/the-textarea-element/textarea-textLength.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-textLength.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>The textLengh IDL attribute</title>
<meta content="charset=utf-16">
<link rel="author" title="tigercosmos" href="mailto:phy.tiger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-textarea-textlength">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<textarea id="textarea"></textarea>
<script>
    var textarea = document.getElementById("textarea");

    test(function () {
        textarea.value= "Hello, World!";
        assert_equals(textarea.textLength, 13);

        textarea.value = "\u4f60\u597d\uff0c\u4e16\u754c\uff01"; //你好，世界!
        assert_equals(textarea.textLength, 6);
    }, "Textarea's 'testLength' should work for utf-16.");
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-textLength.html"
}
```
