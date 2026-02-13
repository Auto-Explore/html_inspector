# html/semantics/interestfor/interestfor-plain-inline-element.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-plain-inline-element.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
  <link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
  <link rel="help" href="https://issues.chromium.org/issues/393408577">
  <link rel="stylesheet" type="text/css" href="/fonts/ahem.css">
  <link rel="match" href="../../../css/reference/ref-filled-green-100px-square.xht">
  <meta name="flags" content="ahem">
  <style>
    a:link {
      color: green;
    }
    [popover] {
      position-area: bottom right;
      inset: auto;
      overflow: visible;
      margin: 0;
      border: none;
      padding: 0;
    }
  </style>
  <script src="/resources/testdriver.js"></script>
  <script src="/resources/testdriver-actions.js"></script>
  <script src="/resources/testdriver-vendor.js"></script>
  <script src="resources/invoker-utils.js"></script>
  <p>Test passes if there is a filled green square and <strong>no red</strong>.</p>
  <div id="container" style="font:20px/1 Ahem; width:100px; height:100px; color:green; background:red;">
    <a id="anchor" href="#" interestfor="doohickey" style="interest-delay:0s;">x</a>xxxx
  </div>
  <div popover="hint" id="doohickey">
    <div style="margin-left:-20px; width:100px; height:80px; background:green;"></div>
  </div>

  <script>
    async function test() {
      await hoverOver(anchor);
      document.documentElement.classList.remove("reftest-wait");
    }
    test();
  </script>
</html>
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
  "source_name": "html/semantics/interestfor/interestfor-plain-inline-element.tentative.html"
}
```
