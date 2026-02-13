# html/semantics/forms/the-option-element/option-text-setter.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-option-element/option-text-setter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<form>
    <select>
        <option value='foo'>bar</option>
    </select>
</form>
<script>
test(() => {
    var option = document.querySelector('option');
    var textChild = option.firstChild;
    assert_equals(textChild.nodeValue, "bar", "Verify that text child node's value equals the option value.");
    assert_true(textChild.isConnected, 'Verify that text child node is in the document.');
    option.text = "baz";
    assert_equals(textChild.nodeValue, "bar", 'Verify that the text child node does not have an updated value.');
    assert_false(textChild.isConnected, 'Verify that the text child node is not in the document.');
    assert_not_equals(textChild, option.firstChild, 'Verify that text child node was replaced by a different text child node.');
    assert_equals(option.firstChild.nodeValue, "baz", 'Verify that the new text child node does equal the updated value.');
    assert_equals(option.text, "baz", 'Verify that option text getter returns the updated value.');
    option.text = "";
    assert_equals(option.firstChild, null, 'Verify that after setting to empty string there are no child text nodes.');
}, 'Verify that using HTMLOptionElement.text setter does not update the existing text child node.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-option-element/option-text-setter.html"
}
```
