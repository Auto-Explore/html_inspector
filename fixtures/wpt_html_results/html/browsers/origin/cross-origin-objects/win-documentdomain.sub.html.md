# html/browsers/origin/cross-origin-objects/win-documentdomain.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/cross-origin-objects/win-documentdomain.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <script src="/common/get-host-info.sub.js"></script>
  <script>
    function loadFrames() {
      window.A = document.getElementById('A').contentWindow;
      window.B = document.getElementById('B').contentWindow;
      window.C = document.getElementById('C').contentWindow;
      window.D = document.getElementById('D').contentWindow;

      var path = location.pathname.substring(0, location.pathname.lastIndexOf('/')) + '/frame.html';
      A.location = 'frame.html';
      B.location = '//{{domains[www2]}}:' + get_port(location) + path;
      C.location = '//{{domains[www2]}}:' + get_port(location) + path;
      D.location = '//{{domains[www1]}}:' + get_port(location) + path;

      var loadCount = 0;
      function frameLoaded() {
        if (++loadCount == 4)
          go();
      }
      var iframes = document.getElementsByTagName('iframe');
      for (var i = 0; i < iframes.length; i++) {
        iframes[i].onload = frameLoaded;
      }
    }

    var results = [];
    function assert(cond, msg) {
      results.push({pass: !!cond, message: msg});
    }

    function go() {
      window.onmessage = function(evt) {
        try {
          assert(evt.data == "PASS", "frame.html processing should be PASS but got " + evt.data);
          assert(B.checkWindowReferences(), "B's Window references are still self-consistent after document.domain");
          for (var i = 0; i < window.length; ++i) {
            assert(window[i] === B.windowReferences[i],
                   "Window reference " + i + " consistent between globals after document.domain");
            assert(window[i].location === B.locationReferences[i],
                   "Location reference " + i + " consistent between globals after document.domain");
          }
        } catch(e) {
          assert(false, "Should not receive exception: " + e);
        }
        opener.postMessage(results, '*');
      };
      A.document.domain = A.document.domain;
      document.domain = document.domain;
      B.postMessage('', '*');
    }

  </script>
</head>
<body onload="loadFrames()">
  <iframe id="A"></iframe>
  <iframe id="B"></iframe>
  <iframe id="C"></iframe>
  <iframe id="D"></iframe>
</body>
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
  "source_name": "html/browsers/origin/cross-origin-objects/win-documentdomain.sub.html"
}
```
