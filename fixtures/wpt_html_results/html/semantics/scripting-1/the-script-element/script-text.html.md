# html/semantics/scripting-1/the-script-element/script-text.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-text.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>HTMLScriptElement.text</title>
<link rel=help href="https://html.spec.whatwg.org/multipage/#dom-script-text">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var script;
setup(function() {
  script = document.createElement("script")
  script.appendChild(document.createComment("COMMENT"))
  script.appendChild(document.createTextNode("  TEXT  "))
  script.appendChild(document.createProcessingInstruction("P", "I"))
  script.appendChild(document.createElement("a"))
        .appendChild(document.createTextNode("ELEMENT"))
})

test(function() {
  assert_equals(script.text, "  TEXT  ")
  assert_equals(script.textContent, "  TEXT  ELEMENT")
}, "Getter")

test(function() {
  script.text = " text "
  assert_equals(script.text, " text ")
  assert_equals(script.textContent, " text ")
  assert_equals(script.firstChild.nodeType, Node.TEXT_NODE)
  assert_equals(script.firstChild.data, " text ")
  assert_equals(script.firstChild, script.lastChild)
  assert_array_equals(script.childNodes, [script.firstChild])
}, "Setter (non-empty string)")

test(function() {
  script.text = ""
  assert_equals(script.text, "")
  assert_equals(script.textContent, "")
  assert_equals(script.firstChild, null)
}, "Setter (empty string)")

test(function() {
  script.text = null
  assert_equals(script.text, "null")
  assert_equals(script.textContent, "null")
  assert_equals(script.firstChild.nodeType, Node.TEXT_NODE)
  assert_equals(script.firstChild.data, "null")
  assert_equals(script.firstChild, script.lastChild)
}, "Setter (null)")

test(function() {
  script.text = undefined
  assert_equals(script.text, "undefined")
  assert_equals(script.textContent, "undefined")
  assert_equals(script.firstChild.nodeType, Node.TEXT_NODE)
  assert_equals(script.firstChild.data, "undefined")
  assert_equals(script.firstChild, script.lastChild)
}, "Setter (undefined)")

test(function() {
  var s = document.createElement("script");
  var text = document.createTextNode("one");
  s.appendChild(text);

  assert_equals(s.firstChild, text);
  assert_equals(text.nodeValue, "one");

  s.text = "two";
  assert_not_equals(s.firstChild, text);
  assert_equals(text.nodeValue, "one");
  assert_equals(s.firstChild.nodeValue, "two");
}, "Setter (text node reuse)")
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-text.html"
}
```
