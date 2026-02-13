# html/browsers/browsing-the-web/navigating-across-documents/refresh/resources/refresh-with-section.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/refresh/resources/refresh-with-section.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<p id="referrer">{{header_or_default(referer, )}}</p>
<section id="section">My section</section>
<span id="info">Refreshing to</span>
<span id=url>{{GET[url]}}</span>
<script>
function refresh() {
  if (url.textContent !== "") {
    const refresh = document.createElement("meta");
    refresh.httpEquiv = "refresh";
    refresh.content = `0; url=${url.textContent}`;
    document.documentElement.appendChild(refresh);
  } else {
    info.textContent = "Not refreshing.";
  }
}

function sendData() {
  const documentReferrer = document.referrer;
  const data = {referrer: referrer.textContent, documentReferrer, url: location.href};
  window.parent.postMessage(data, "*");
}

const sectionHash = "#section";
if (url.textContent !== sectionHash) {
  window.addEventListener("hashchange", refresh);
  location.hash = sectionHash;
} else if (location.hash !== sectionHash) {
  window.addEventListener("hashchange", sendData);
  refresh();
} else {
  sendData();
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 135,
        "byte_start": 125,
        "col": 33,
        "line": 4
      }
    },
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/refresh/resources/refresh-with-section.sub.html"
}
```
