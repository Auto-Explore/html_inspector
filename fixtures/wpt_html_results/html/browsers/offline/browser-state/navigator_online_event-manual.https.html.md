# html/browsers/offline/browser-state/navigator_online_event-manual.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/offline/browser-state/navigator_online_event-manual.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
 <head>
  <title>Offline Application Cache</title>
  <link rel="stylesheet" href="../resources/css/result.css">
 </head>
 <body>
  <h1>navigator_online_event</h1>

  <ol>
  <li>Change the 'work offline' mode.</li>
  <li>If actual result and expected result are same, then test is <span class="manualpass">Pass</span>, otherwise <span class="manualfail">Fail</span>.</li>
  </ol>

  <hr>

  <h2>Actual Result</h2>
  <div id="actualResult">
    <span id="actualMsg"></span>
  </div>

  <h2>Expected Result</h2>
  <div id="expectedResult">
    <span id="expectedMsg">apply 'work offline': offline event is raised.<p>release 'work offline': online event is raised.</span>
  </div>
  <script>

  function showOnline(e) {
    let msg = 'online event is raised';
    if (e.target != window)
      msg += ' (on the WRONG target)';
    document.getElementById('actualMsg').innerHTML = msg + '.';
  }

  function showOffline(e) {
    let msg = 'offline event is raised';
    if (e.target != window)
      msg += ' (on the WRONG target)';
    document.getElementById('actualMsg').innerHTML = msg + '.';
  }

  window.addEventListener("online", showOnline, false);
  window.addEventListener("offline", showOffline, false);
  </script>
 </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 636,
        "byte_start": 633,
        "col": 74,
        "line": 24
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
  "source_name": "html/browsers/offline/browser-state/navigator_online_event-manual.https.html"
}
```
