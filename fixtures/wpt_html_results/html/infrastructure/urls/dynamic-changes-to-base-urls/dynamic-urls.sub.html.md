# html/infrastructure/urls/dynamic-changes-to-base-urls/dynamic-urls.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/dynamic-changes-to-base-urls/dynamic-urls.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>HTML Test: Dynamic changes to base URLs</title>
<link rel="author" title="Intel" href="http://www.intel.com/"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dynamic-changes-to-base-urls" />
<base href="" id="base">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="div" style="display:none"></div>
<script>
      var div = document.getElementById("div"),
          base = document.getElementById("base"),
          url =  document.location.href;

      var testData = [
        {elements: ["a", "link", "area"], set: "href", get: "href"},
        {elements: ["q", "blockquote", "ins", "del"], set: "cite", get: "cite"},
        {elements: ["audio", "input", "img", "embed", "video", "iframe", "script", "source", "track"], set: "src", get: "src"},
        {elements: ["form"], set: "action", get: "action"},
        {elements: ["object"], set: "data", get: "data"},
        {elements: ["button"], set: "formAction", get: "formAction"}
      ];

      for (var i in testData) {
        var item = testData[i];
        for (var j in item.elements) {
          test(function () {
            var ele = document.createElement(item.elements[j]);

            ele.setAttribute(item.set, "test.txt");
            div.appendChild(ele);

            base.setAttribute("href", "");
            assert_equals(ele[item.get], url.substr(0, url.lastIndexOf("/")) +"/test.txt", "The '" + item.get + "' attribute is incorrect.");
            base.setAttribute("href", "http://{{domains[www]}}:{{ports[http][0]}}");
            assert_equals(ele[item.get], "http://{{domains[www]}}:{{ports[http][0]}}/test.txt", "The '" + item.get + "' attribute is incorrect.");
          }, "The '" + item.set + "' attribute of the '" + item.elements[j] + "' element");
        }
      }
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 276,
        "byte_start": 252,
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
  "source_name": "html/infrastructure/urls/dynamic-changes-to-base-urls/dynamic-urls.sub.html"
}
```
