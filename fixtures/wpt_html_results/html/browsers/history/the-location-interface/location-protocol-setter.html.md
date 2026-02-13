# html/browsers/history/the-location-interface/location-protocol-setter.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location-protocol-setter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Set location.protocol to schemes that throw</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<iframe src="data:text/html,<script>
onmessage = (e) => {
  let results = [];
  e.data.forEach((val) => {
    try {
      location.protocol = val;
      results.push('failure')
    } catch(e) {
      results.push(e.name)
    }
  });
  parent.postMessage(results, '*')
}
</script>"></iframe>
<iframe srcdoc="<script>
onmessage = (e) => {
  let results = [];
  e.data.forEach((val) => {
    try {
      location.protocol = val;
      results.push('failure')
    } catch(e) {
      results.push(e.name)
    }
  });
  parent.postMessage(results, '*')
}
</script>"></iframe>
<script>
  const broken = [
    '\x00',
    '\x01',
    '\x09', // becomes the empty string
    '\x0A', // becomes the empty string
    '\x0C',
    '\x0D',
    '\x20',
    '\x21',
    '\x7F',
    '\x80',
    '\xFF',
    ':',
    '†',
    '\x00x',
    '\x01x',
    '\x20x',
    '\x21x',
    '\x7Fx',
    '\x80x',
    '\xFFx',
    ':x',
    '†x',
    '\x00X',
    '\x01X',
    '\x20X',
    '\x21X',
    '\x7FX',
    '\x80X',
    '\xFFX',
    ':X',
    '†X',
    'x\x00',
    'x\x01',
    'x\x20',
    'x\x21',
    'x\x7F',
    'x\x80',
    'x\xFF',
    'x†',
    'X\x00',
    'X\x01',
    'X\x20',
    'X\x21',
    'X\x7F',
    'X\x80',
    'X\xFF',
    'X†',
  ];

  broken.forEach(val => {
    test(() => {
      assert_throws_dom("SyntaxError", () => { location.protocol = val })
    }, `${encodeURI(val)} (percent-encoded here for clarity) is not a scheme`)
  })
  let c = 0
  async_test((t) => {
    self.onload = t.step_func(() => {
      self.onmessage = t.step_func((e) => {
        assert_array_equals(e.data, broken.map(() => "SyntaxError"))
        c++
        if(c === 2) {
          t.done()
        }
      })
      self[0].postMessage(broken, "*")
      self[1].postMessage(broken, "*")
    })
  }, "Equivalent tests for data URL and srcdoc <iframe>s")
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “data:text/html,<script>\nonmessage = (e) => {\n  let results = [];\n  e.data.forEach((val) => {\n    try {\n      location.protocol = val;\n      results.push('failure')\n    } catch(e) {\n      results.push(e.name)\n    }\n  });\n  parent.postMessage(results, '*')\n}\n</script>” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 477,
        "byte_start": 196,
        "col": 1,
        "line": 6
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
  "source_name": "html/browsers/history/the-location-interface/location-protocol-setter.html"
}
```
