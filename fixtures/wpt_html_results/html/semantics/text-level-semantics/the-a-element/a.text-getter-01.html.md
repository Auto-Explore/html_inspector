# html/semantics/text-level-semantics/the-a-element/a.text-getter-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-a-element/a.text-getter-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLAnchorElement.text getting</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-a-text">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>var b</script>
<div id="test">
<a href="a">a b c </a>
<a href="b">a <!--b-->b c </a>
<a href="c">a <b>b</b> c </a>
<a href="d">a <script>b</script> c </a>
<script>
var e = document.getElementById("test")
                .appendChild(document.createElement("a"));
e.href = "d";
e.appendChild(document.createTextNode("a "));
e.appendChild(document.createTextNode("b "));
e.appendChild(document.createTextNode("c "));
</script>
</div>
<script>
test(function() {
  var list = document.getElementById("test")
                     .getElementsByTagName("a");
  for (var i = 0, il = list.length; i < il; ++i) {
    test(function() {
      assert_equals(list[i].text, list[i].textContent);
      assert_equals(list[i].text, "a b c ");
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
  "source_name": "html/semantics/text-level-semantics/the-a-element/a.text-getter-01.html"
}
```
