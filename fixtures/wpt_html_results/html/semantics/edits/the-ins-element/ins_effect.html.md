# html/semantics/edits/the-ins-element/ins_effect.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/edits/the-ins-element/ins_effect.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=UTF-8>
<title>HTML Test: Text in the ins element should be 'underline'</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-ins-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<p><ins>underlined text</ins></p>
<div id="log"></div>

<script>
  test(function() {
    var element = document.getElementsByTagName('ins')[0],
        textDecoration = getComputedStyle(element).textDecorationLine ||
                         getComputedStyle(element).textDecoration;
    assert_equals(textDecoration, 'underline');
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
  "source_name": "html/semantics/edits/the-ins-element/ins_effect.html"
}
```
