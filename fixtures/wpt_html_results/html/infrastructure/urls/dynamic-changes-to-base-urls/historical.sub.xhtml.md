# html/infrastructure/urls/dynamic-changes-to-base-urls/historical.sub.xhtml

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/dynamic-changes-to-base-urls/historical.sub.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html id="h" xmlns="http://www.w3.org/1999/xhtml" xml:base="">
  <head>
    <title>Historical xml:base must be removed</title>
    <link rel="author" title="Intel" href="http://www.intel.com/"/>
    <link rel="help" href="https://github.com/whatwg/html/pull/84/files" />
    <script src="/resources/testharness.js" id="s1"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <div id="div" style="display:none" xml:base=""></div>
    <script>
      <![CDATA[
      var div = document.getElementById("div"),
          html = document.getElementById("h"),
          url =  document.location.href;

      var testData = [
        {elements: ["a", "link", "area"], set: "href", get: "href"},
        {elements: ["q", "blockquote", "ins", "del"], set: "cite", get: "cite"},
        {elements: ["audio", "input", "img", "embed", "video", "iframe", "script", "source", "track"], set: "src", get: "src"},
        {elements: ["form"], set: "action", get: "action"},
        {elements: ["object"], set: "data", get: "data"},
        {elements: ["button"], set: "formaction", get: "formAction"}
      ];

      for (var i in testData) {
        var item = testData[i];
        for (var j in item.elements) {
          test(function () {
            var ele = document.createElement(item.elements[j]);

            ele.setAttribute(item.set, "test.txt");
            div.appendChild(ele);

            html.setAttribute("xml:base", "");
            assert_equals(ele[item.get], url.substr(0, url.lastIndexOf("/")) +"/test.txt", "The '" + item.get + "' attribute is incorrect.");
            html.setAttribute("xml:base", "http://{{domains[www]}}:{{ports[http][0]}}");
            assert_not_equals(ele[item.get], "http://{{domains[www]}}:{{ports[http][0]}}/test.txt", "The '" + item.get + "' attribute is incorrect.");
          }, "The '" + item.set + "' attribute of the '" + item.elements[j] + "' element");
        }
      }

      test(function () {
        var s1 = document.getElementById("s1");
        var val1 = s1.src;
        var val2 = div.firstChild.href;

        div.setAttribute("xml:base", "http://{{domains[www2]}}:{{ports[http][0]}}");
        assert_equals(s1.src, val1, "The outer element must not be effected.");
        assert_equals(div.firstChild.href, val2, "The inner element must be effected.");
        assert_not_equals(div.firstChild.href, "http://{{domains[www2]}}:{{ports[http][0]}}/test.txt");
      }, "Change the base URL must not effect the descendant elements");
      ]]>
    </script>
  </body>
</html>
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
  "source_name": "html/infrastructure/urls/dynamic-changes-to-base-urls/historical.sub.xhtml"
}
```
