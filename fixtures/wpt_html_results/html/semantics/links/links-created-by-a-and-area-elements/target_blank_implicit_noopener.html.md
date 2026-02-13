# html/semantics/links/links-created-by-a-and-area-elements/target_blank_implicit_noopener.html

Counts:
- errors: 5
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/links/links-created-by-a-and-area-elements/target_blank_implicit_noopener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <meta charset=utf-8>
  <meta name="timeout" content="long">
  <title>Test behavior of target=_blank links</title>
  <script src=/resources/testharness.js></script>
  <script src=/resources/testharnessreport.js></script>
</head>
<body>
  <a href="support/target_blank_implicit_noopener.html?a1" id="a1" rel="noopener" target="_blank">Click me</a>
  <a href="support/target_blank_implicit_noopener.html?a2" id="a2" rel="opener" target="_blank">Click me</a>
  <a href="support/target_blank_implicit_noopener.html?a3" id="a3" target="_blank">Click me</a>
  <a href="support/target_blank_implicit_noopener.html?a4" id="a4" rel="opener noopener" target="_blank">Click me</a>
  <a href="support/target_blank_implicit_noopener.html?a5" id="a5" rel="noopener opener" target="_blank">Click me</a>
  <a href="support/target_blank_implicit_noopener.html?a6" id="a6" rel="noreferrer" target="_blank">Click me</a>
  <a href="support/target_blank_implicit_noopener.html?a7" id="a7" rel="opener noreferrer" target="_blank">Click me</a>
  <a href="support/target_blank_implicit_noopener.html?a8" id="a8" rel="noopener opener noreferrer" target="_blank">Click me</a>

  <!-- Although this is not valid, per the processing model of area it ought to work -->
  <area shape="rect" coords="0,0,99,50" href="support/target_blank_implicit_noopener.html?area1" id="area1" rel="noopener" target="_blank">
  <area shape="rect" coords="0,0,99,50" href="support/target_blank_implicit_noopener.html?area2" id="area2" rel="opener" target="_blank">
  <area shape="rect" coords="0,0,99,50" href="support/target_blank_implicit_noopener.html?area3" id="area3" target="_blank">
  <area shape="rect" coords="0,0,99,50" href="support/target_blank_implicit_noopener.html?area4" id="area4" rel="opener noopener" target="_blank">
  <area shape="rect" coords="0,0,99,50" href="support/target_blank_implicit_noopener.html?area5" id="area5" rel="noopener opener" target="_blank">

  <script>

  let tests = [
    { id: "a1", hasOpener: false, name: "Anchor element with target=_blank with rel=noopener" },
    { id: "a2", hasOpener: true, name: "Anchor element with target=_blank with rel=opener" },
    { id: "a3", hasOpener: false, name: "Anchor element with target=_blank with implicit rel=noopener" },
    { id: "a4", hasOpener: false, name: "Anchor element with target=_blank with rel=opener+noopener" },
    { id: "a5", hasOpener: false, name: "Anchor element with target=_blank with rel=noopener+opener" },
    { id: "a6", hasOpener: false, name: "Anchor element with target=_blank with rel=noreferrer" },
    { id: "a7", hasOpener: false, name: "Anchor element with target=_blank with rel=opener+noreferrer" },
    { id: "a8", hasOpener: false, name: "Anchor element with target=_blank with rel=noopener+opener+noreferrer" },
    { id: "area1", hasOpener: false, name: "Area element with target=_blank with rel=noopener" },
    { id: "area2", hasOpener: true, name: "Area element with target=_blank with rel=opener" },
    { id: "area3", hasOpener: false, name: "Area element with target=_blank with implicit rel=noopener" },
    { id: "area4", hasOpener: false, name: "Area element with target=_blank with rel=opener+noopener" },
    { id: "area5", hasOpener: false, name: "Area element with target=_blank with rel=noopener+opener" },
  ];

  tests.forEach(data => {
    async_test(
      test => {
        let bc = new BroadcastChannel(data.id);
        bc.addEventListener("message", test.step_func_done(e => {
          assert_equals(e.data.hasOpener, data.hasOpener);
        }), {once: true});

        document.getElementById(data.id).click();
      }, data.name);
  });
  </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 1410,
        "byte_start": 1273,
        "col": 3,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 1548,
        "byte_start": 1413,
        "col": 3,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 1673,
        "byte_start": 1551,
        "col": 3,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 1820,
        "byte_start": 1676,
        "col": 3,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.area.map_ancestor.missing",
      "message": "The “area” element must have a “map” ancestor.",
      "severity": "Error",
      "span": {
        "byte_end": 1967,
        "byte_start": 1823,
        "col": 3,
        "line": 25
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/links/links-created-by-a-and-area-elements/target_blank_implicit_noopener.html"
}
```
