# html/browsers/windows/consume-user-activation/support/incumbent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/consume-user-activation/support/incumbent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Incumbent page used as a test helper</title>
<button id="focus-opener-button" onclick="opener.focus()">Focus opener</button>
<script>
'use strict';

function pageDone(expectedMessage) {
  return new Promise(resolve => {
    window.addEventListener('message', e => {
      if (e.data === expectedMessage) {
        resolve();
      }
    });
  });
}

onload = async () => {
  await opener.test_driver.bless("open current popup", null, window);
  const currentDone = pageDone("current page");
  const currentWin = window.open("current.html", "_blank");
  await currentDone;
  await opener.test_driver.bless("open relevant popup", null, window);
  const relevantDone = pageDone("relevant page");
  const relevantWin = window.open("relevant.html", "_blank");
  await relevantDone;
  window.openTestPopup = function() {
    // This is the multi-global incarnation
    return currentWin.open.call(relevantWin, "/resources/blank.html", "_blank");
  };
  opener.currentWin = currentWin;
  opener.relevantWin = relevantWin;
  await opener.test_driver.click(document.getElementById("focus-opener-button"));
  opener.postMessage("incumbent page", "*");
};
</script>
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
  "source_name": "html/browsers/windows/consume-user-activation/support/incumbent.html"
}
```
