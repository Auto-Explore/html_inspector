# html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-02.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <meta charset="utf-8">
  <title>HTML Test: the browsing contexts created by various container elements</title>
  <link rel="author" title="Intel" href="http://www.intel.com/" />
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script>

  var t1 = async_test("Accessing child browsing contexts 1");
  var t2 = async_test("Accessing child browsing contexts 2");
  var t3 = async_test("Accessing child browsing contexts 3");
  function on_load() {
    //Child browsing contexts created by iframe, object and embed elements.
    t1.step(function () {
      assert_equals(window.length, 3, "The top browsing context should have 3 child browsing contexts.");
    });
    t1.step(function () {
      assert_equals(window[0].name, "win1", "The browsing context name should be 'win1'.");
      assert_equals(window[1].name, "win2", "The browsing context name should be 'win2'.");
      assert_equals(window[2].name, "win3", "The browsing context name should be 'win3'.");
    });
    t1.done();

    //Child browsing contexts created by frame elements.
    t2.step(function () {
      assert_equals(document.getElementById("fr").contentWindow.length, 2,
                    "The child browsing context created by the iframe element should have 2 child browsing contexts.");
    });
    t2.step(function () {
      assert_equals(document.getElementById("fr").contentWindow[0].name, "win4",
                    "The browsing context name should be 'win4'.");
      assert_equals(document.getElementById("fr").contentWindow[1].name, "win5",
                    "The browsing context name should be 'win5'.");
    });
    t2.done();

    //The child browsing context will be removed if the data attribute of the associated object element is removed.
    t3.step(function () {
      document.getElementById("obj").removeAttribute("type");
      assert_equals(window.length, 3, "The top browsing context should have 3 child browsing contexts.");
      document.getElementById("obj").removeAttribute("data");
      assert_equals(window.length, 3, "The top browsing context should have 3 child browsing contexts.");

      setTimeout(function () {
        assert_equals(window.length, 2, "The top browsing context should have 2 child browsing contexts.");
      }, 1);
    });
    t3.done();
  }

  </script>
</head>
<body onload="on_load()">
  <div id="log"></div>
  <div style="display:none">
    <iframe id="fr" name="win1" src="test3.html"></iframe>
    <object id="obj" name="win2" type="text/html" data="about:blank"></object>
    <object type="image/png" src="/images/green.png"></object>
    <embed id="emb" name="win3" type="image/svg+xml" src="/images/green.svg"></embed>
  </div>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2658,
        "byte_start": 2609,
        "col": 5,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 2753,
        "byte_start": 2745,
        "col": 78,
        "line": 60
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
  "source_name": "html/browsers/the-window-object/accessing-other-browsing-contexts/indexed-browsing-contexts-02.html"
}
```
