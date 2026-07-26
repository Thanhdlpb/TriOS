let editor;

document.addEventListener('DOMContentLoaded', function() {
    editor = CodeMirror.fromTextArea(document.getElementById('codeEditor'), {
        mode: 'text',
        theme: 'monokai',
        lineNumbers: true,
        indentUnit: 4,
        tabSize: 4,
        lineWrapping: true
    });
});

async function runCode() {
    const code = editor.getValue();
    const output = document.getElementById('output');
    const status = document.getElementById('status');
    
    output.innerHTML = '<span class="info">Đang chạy...</span>';
    status.textContent = 'Đang thực thi...';
    
    try {
        const response = await fetch('/api/run', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ code: code })
        });
        
        const data = await response.json();
        
        if (data.error) {
            output.innerHTML = `<span class="error">Lỗi: ${data.error}</span>`;
            status.textContent = 'Lỗi';
        } else {
            output.innerHTML = data.output.map(line => 
                `<span class="success">${escapeHtml(line)}</span>`
            ).join('\n');
            status.textContent = '✅ Thành công';
        }
    } catch (e) {
        output.innerHTML = `<span class="error">Lỗi kết nối: ${e.message}</span>`;
        status.textContent = 'Lỗi kết nối';
    }
}

async function saveCode() {
    const code = editor.getValue();
    const fileName = prompt('Nhập tên file (.tri):', 'chuong_trinh.tri');
    if (!fileName) return;
    
    try {
        const response = await fetch('/api/save', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ fileName, code })
        });
        
        const data = await response.json();
        document.getElementById('status').textContent = data.message || 'Đã lưu';
    } catch (e) {
        document.getElementById('status').textContent = 'Lỗi lưu file';
    }
}

function openFile() {
    const fileInput = document.getElementById('fileInput');
    const file = fileInput.files[0];
    if (!file) return;
    
    const reader = new FileReader();
    reader.onload = function(e) {
        editor.setValue(e.target.result);
        document.getElementById('status').textContent = `Đã mở: ${file.name}`;
    };
    reader.readAsText(file);
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
