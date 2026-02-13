# html/semantics/selectors/pseudo-classes/focus.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/focus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Selector: pseudo-classes (:focus)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org" id=link1>
<link rel=help href="https://html.spec.whatwg.org/multipage/#pseudo-classes" id=link2>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="utils.js"></script>
<body id=body tabindex=0>
  <div id="log"></div>
  <button id=button1 type=submit>button1</button>
  <input id=input1>
  <input id=input2 disabled>
  <textarea id=textarea1>textarea1</textarea>
  <input type=checkbox id=checkbox1 checked>
  <input type=radio id=radio1 checked>
  <div tabindex=0 id=div1>hello</div>
  <div contenteditable id=div2>content</div>
  <iframe src="focus-iframe.html" id=iframe></iframe>

  <script>
    setup({explicit_done: true});

    onload = function() {
      if (document.hasFocus() || frames[0].document.hasFocus()) {
        run_test()
      } else {
        window.onfocus = run_test;
      }
    }

    function run_test() {
      document.getElementById("input1").focus(); // set the focus on input1
      testSelectorIdsMatch(":focus", ["input1"], "input1 has the focus");

      document.getElementById("div1").focus();
      testSelectorIdsMatch(":focus", ["div1"], "tabindex attribute makes the element focusable");

      document.getElementById("div2").focus();
      testSelectorIdsMatch(":focus", ["div2"], "editable elements are focusable");

      document.body.focus();
      testSelectorIdsMatch(":focus", ["body"], "':focus' matches focussed body with tabindex");

      document.getElementById("iframe").contentDocument.getElementById("inputiframe").focus();
      testSelectorIdsMatch(":focus", [], "':focus' doesn't match focused elements in iframe");

      done();
    }
  </script>
</body>
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
  "source_name": "html/semantics/selectors/pseudo-classes/focus.html"
}
```
