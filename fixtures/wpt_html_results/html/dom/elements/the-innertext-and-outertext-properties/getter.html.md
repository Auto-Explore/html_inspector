# html/dom/elements/the-innertext-and-outertext-properties/getter.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/the-innertext-and-outertext-properties/getter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>innerText/outerText getter test</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
.before::before { content:'abc'; }
.table { display:table; }
.itable { display:inline-table; }
.row { display:table-row; }
.cell { display:table-cell; }
.first-line-uppercase::first-line { text-transform:uppercase; }
.first-letter-uppercase::first-letter { text-transform:uppercase; }
.first-letter-float::first-letter { float:left; }
</style>
<div id="container"></div>
<svg id="svgContainer"></svg>
<script>
let container = document.querySelector('#container');
let svgContainer = document.querySelector('#svgContainer');
function testText(html, expectedPlain, msg) {
  textTextInContainer(container, html, expectedPlain, msg);
}
function testTextInSVG(html, expectedPlain, msg) {
  textTextInContainer(svgContainer, html, expectedPlain, msg);
}
function textTextInContainer(cont, html, expectedPlain, msg) {
  test(function() {
    container.innerHTML = html;
    if (cont != container) {
      while (container.firstChild) {
        cont.appendChild(container.firstChild);
      }
    }
    var e = document.getElementById('target');
    if (!e) {
      e = cont.firstChild;
    }
    var pokes = document.getElementsByClassName('poke');
    for (var i = 0; i < pokes.length; ++i) {
      pokes[i].textContent = 'abc';
    }
    ['rp', 'optgroup', 'div'].forEach(function(tag) {
      pokes = document.getElementsByClassName('poke-' + tag);
      for (var i = 0; i < pokes.length; ++i) {
        var el = document.createElement(tag);
        el.textContent = "abc";
        pokes[i].appendChild(el);
      }
    });
    var shadows = document.getElementsByClassName('shadow');
    for (var i = 0; i < shadows.length; ++i) {
      var s = shadows[i].attachShadow({ mode: "open" });
      s.textContent = 'abc';
    }
    while (e && e.nodeType != Node.ELEMENT_NODE) {
      e = e.nextSibling;
    }
    assert_equals(e.innerText, expectedPlain, "innerText");
    assert_equals(e.outerText, expectedPlain, "outerText");
    cont.textContent = '';
  }, msg + ' (' + format_value(html) + ')');
}
</script>
<script src="getter-tests.js"></script>
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
  "source_name": "html/dom/elements/the-innertext-and-outertext-properties/getter.html"
}
```
