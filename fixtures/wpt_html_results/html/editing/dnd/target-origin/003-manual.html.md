# html/editing/dnd/target-origin/003-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/003-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>allowTargetOrigin valid syntax</title>
    <style type="text/css">
div { float: left; height: 100px; width: 100px; margin-right: 10px; background: orange; }
iframe { height: 100px; width: 100px; border: none; }
.note { float: right; color: silver; }
    </style>
    <script type="text/javascript" src="../resources/crossorigin.sub.js"></script>
    <script type="text/javascript">
function addNote(el,str) {
  var par = document.createElement(el);
  par.textContent = str;
  document.body.appendChild(par);
}
function testFrame(text,frameorigin,framepath) {
  var persist = arguments;
  addNote('p',(done++)+'. '+text);
  var div = document.createElement('div');
  var frame = document.createElement('iframe');
  frame.src = frameorigin+framepath;
  div.draggable = true;
  div.ondragstart = function (e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text','dummy text');
    for( var i = 3; i < persist.length; i++ ) {
      e.dataTransfer.allowTargetOrigin(persist[i]);
    }
  };
  var par = document.createElement('p');
  par.className = 'note';
  par.appendChild(document.createTextNode('Target: '+frameorigin));
  par.appendChild(document.createElement('br'));
  par.appendChild(document.createTextNode('Allowing: '+([]).slice.call(persist,3).join(' and ')));
  if( framepath.match(/\?domain\b/) ) {
    par.appendChild(document.createElement('br'));
    par.appendChild(document.createTextNode('document.domain set to parent domain'));
  }
  document.body.appendChild(par);
  document.body.appendChild(div);
  document.body.appendChild(frame);
}
var done = 1;
window.onload = function () {
  var allowText = 'Drag the orange box below over the blue box the right, and release it. Fail if nothing happens in the blue box.';
  var blockText = 'Drag the orange box below over the pink box the right, and release it. Pass if nothing happens in the pink box.';
  var allowHelper = location.pathname.replace(/[^\/]*$/,'HELPER-mustallow.html');
  var blockHelper = location.pathname.replace(/[^\/]*$/,'HELPER-mustblock.html');
  if( location.hostname != httpHostMain || location.host != httpHostMain ) {
    addNote('p','This test must be loaded over http:\/\/'+httpHostMain+'\/');
  } else {
    /* 01 */ testFrame(allowText,'http://'+httpHostMain,allowHelper);
    /* 02 */ testFrame(allowText,'http://'+httpHostAlias,allowHelper);
    /* 03 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'*');
    /* 04 */ testFrame(allowText,'http://'+httpHostAlias,allowHelper,'*');
    /* 05 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'/');
    /* 06 */ testFrame(blockText,'http://'+httpHostAlias,blockHelper,'/');
    /* 07 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'http://'+httpHostMain);
    /* 08 */ testFrame(blockText,'http://'+httpHostMain+':'+httpPortAlias,blockHelper,'http://'+httpHostMain);
    /* 09 */ testFrame(blockText,'http://'+httpHostAlias,blockHelper,'http://'+httpHostMain);
    /* 10 */ testFrame(blockText,'http://'+httpHostAlias,blockHelper,'http://'+httpHostMain);
    /* 11 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'http://'+httpHostMain+':80');
    /* 12 */ testFrame(blockText,'http://'+httpHostMain+':'+httpPortAlias,blockHelper,'http://'+httpHostMain+':80');
    /* 13 */ testFrame(allowText,'http://'+httpHostMain+':'+httpPortAlias,allowHelper,'http://'+httpHostMain+':'+httpPortAlias);
    /* 14 */ testFrame(blockText,'http://'+httpHostMain,blockHelper,'http://'+httpHostMain+':'+httpPortAlias);
    /* 15 */ testFrame(blockText,'https://'+httpsHostAlias,blockHelper,'http://'+httpsHostAlias);
    /* 16 */ testFrame(allowText,'https://'+httpsHostAlias,allowHelper,'https://'+httpsHostAlias);
    /* 17 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'http://foo:bar@'+httpHostMain+'/baz');
    /* 18 */ testFrame(allowText,'http://foo:bar@'+httpHostMain,allowHelper,'http://'+httpHostMain);
    /* 19 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'http://'+httpHostAlias,'/');
    /* 20 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'/','http://'+httpHostAlias);
    /* 21 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'http://'+httpHostAlias,'*');
    /* 22 */ testFrame(allowText,'http://'+httpHostMain,allowHelper,'http://'+httpHostAlias,'http://'+httpHostMain);
    /* 23 */ testFrame(allowText,'http://'+httpHostAlias,allowHelper,'http://'+httpHostAlias,'http://'+httpHostMain);
    /* 24 */ testFrame(blockText,'http://'+httpHostAlias,blockHelper,'http://dummy','http://'+httpHostMain);
    /* 25 */ testFrame(blockText,'https://'+httpsHostAlias,blockHelper,'https://'+httpsHostAlias+':'+httpsPortAlias);
    /* 26 */ testFrame(blockText,'https://'+httpsHostAlias+':'+httpsPortAlias,blockHelper,'https://'+httpsHostAlias);
    /* 27 */ testFrame(allowText,'https://'+httpsHostAlias+':'+httpsPortAlias,allowHelper,'https://'+httpsHostAlias+':'+httpsPortAlias);
    window.xhr = new XMLHttpRequest();
    xhr.open('GET',allowHelper,false);
    xhr.send(null);
    /* 28 */ testFrame(allowText,'data:text/html,',escape(xhr.responseText),'http://'+httpHostMain);
    /* 29 */ testFrame(allowText,'javascript:','parent.xhr.responseText','http://'+httpHostMain);
    /* 30 */ testFrame(blockText,'http://'+httpHostAlias,blockHelper,'http://'+httpHostAlias.replace(/^[^.]+\./,''));
    /* 31 */ testFrame(allowText,'http://'+httpHostAlias,allowHelper+'?domain','http://'+httpHostAlias);
    /* 32 */ testFrame(blockText,'http://'+httpHostAlias,blockHelper+'?domain','http://'+httpHostAlias.replace(/^[^.]+\./,''));
  }
};
    </script>
  </head>
  <body>
    <noscript><p>Enable JavaScript and reload</p></noscript>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 109,
        "byte_start": 86,
        "col": 5,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 379,
        "byte_start": 310,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 424,
        "byte_start": 393,
        "col": 5,
        "line": 11
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
  "source_name": "html/editing/dnd/target-origin/003-manual.html"
}
```
