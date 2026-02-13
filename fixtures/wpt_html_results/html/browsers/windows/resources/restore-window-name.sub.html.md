# html/browsers/windows/resources/restore-window-name.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/resources/restore-window-name.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
<title>popup helper for restore window.name test</title>
<script>

const search = window.location.search.replace("?", "");
const name = search.split("=")[1];

if (!search.startsWith("name=")) {
  throw new Error("Unsupported test!");
}

function startTest() {
  if (window.name === "start") {
    window.name = name;
    const url = new URL(window.location);
    url.host = "{{hosts[alt][]}}:{{ports[https][0]}}";
    url.pathname = url.pathname.slice(0, url.pathname.lastIndexOf("/") + 1);
    url.pathname += "restore-window-name-back.sub.html";
    window.location = url.href;
  }
}

function verifyResult() {
  const result = document.getElementById("result");
  if (window.name === "start") {
    result.textContent = "Please first click the above 'start test' link.";
    return;
  }

  result.textContent = window.name === name ? "PASS" : "FAIL";
}

</script>
</head>
<body>
  <p>Please first click the 'start test' link. And then, click the 'Verify Result' button to verify the result. You should get a PASS after clicking the button</p>
  <a id="navigate" href="javascript:void(0)" onclick="startTest()">start test</a><br>
  <button onclick="verifyResult()">Verify Result</button>
  Result: <a id="result"></a>
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
  "source_name": "html/browsers/windows/resources/restore-window-name.sub.html"
}
```
