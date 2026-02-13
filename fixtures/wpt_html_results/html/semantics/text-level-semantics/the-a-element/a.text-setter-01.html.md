# html/semantics/text-level-semantics/the-a-element/a.text-setter-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-a-element/a.text-setter-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLAnchorElement.text setting</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-a-text">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<div id="test">
<a href="a">a b c</a>
<a href="b">a <!--b--> c</a>
<a href="c">a <b>b</b> c</a>
<script>
var d = document.getElementById("test")
                .appendChild(document.createElement("a"));
d.href = "d";
d.appendChild(document.createTextNode("a "));
d.appendChild(document.createTextNode("b "));
d.appendChild(document.createTextNode("c "));
</script>
</div>
<script>
test(function() {
  var list = document.getElementById("test")
                     .getElementsByTagName("a");
  for (var i = 0, il = list.length; i < il; ++i) {
    test(function() {
      list[i].text = "x";
      assert_equals(list[i].text, "x");
      assert_equals(list[i].textContent, "x");
      assert_equals(list[i].firstChild.data, "x");
      assert_equals(list[i].childNodes.length, 1);

      list[i].textContent = "y";
      assert_equals(list[i].text, "y");
      assert_equals(list[i].textContent, "y");
      assert_equals(list[i].firstChild.data, "y");
      assert_equals(list[i].childNodes.length, 1);
    }, "Test for anchor " + i);
  }
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
  "source_name": "html/semantics/text-level-semantics/the-a-element/a.text-setter-01.html"
}
```
