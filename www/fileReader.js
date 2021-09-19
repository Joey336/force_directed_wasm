
function init(){
    document.getElementById('fileInput').addEventListener('change', handleFileSelect, false);
  }
  
  function handleFileSelect(event){
    const reader = new FileReader()
    reader.onload = handleFileLoad;
    reader.readAsText(event.target.files[0])
  }
  
  function handleFileLoad(event){
    document.getElementById('fileContent').textContent = event.target.result;
    return document.getElementById('fileContent').textContent;
    
  }