#!/bin/bash

# 数据文件名
data_file="image_data.txt"

# 检查数据文件是否存在
if [ ! -f "$data_file" ]; then
    echo "错误: 找不到数据文件 $data_file"
    echo "请确保 $data_file 与脚本在同一目录下"
    exit 1
fi

# 创建下载目录
download_dir="downloaded_images"
mkdir -p "$download_dir"

# 计数器
count=0
success_count=0
fail_count=0

echo "开始下载图片..."
echo "===================================="

# 读取数据文件
while IFS= read -r line; do
    # 跳过空行
    if [[ -z "$line" ]]; then
        continue
    fi
    
    # 如果是偶数行（从0开始），则是URL
    if (( count % 2 == 0 )); then
        url="$line"
    else
        name="$line"
        
        # 获取文件扩展名
        extension="${url##*.}"
        
        # 清理文件名（移除可能引起问题的字符）
        safe_name=$(echo "$name" | tr -d '/<>:\"\\|?*' | sed 's/ /_/g')
        
        # 构建文件名
        filename="${safe_name}.${extension}"
        filepath="${download_dir}/${filename}"
        
        echo "正在下载: $name"
        echo "URL: $url"
        echo "保存为: $filename"
        
        # 使用curl下载文件
        if curl -s -f -L -o "$filepath" "$url"; then
            echo "✓ 下载成功"
            ((success_count++))
        else
            echo "✗ 下载失败"
            # 删除可能的部分下载文件
            rm -f "$filepath"
            ((fail_count++))
        fi
        echo "------------------------------------"
    fi
    
    ((count++))
    
done < "$data_file"

echo "===================================="
echo "下载完成！"
echo "成功: $success_count 个文件"
echo "失败: $fail_count 个文件"
echo "文件保存在: $download_dir 目录中"

# 检查是否真的下载了文件
if [ $success_count -eq 0 ]; then
    echo ""
    echo "警告: 没有成功下载任何文件！"
    echo "请检查:"
    echo "1. 网络连接是否正常"
    echo "2. URL地址是否有效"
    echo "3. 数据文件格式是否正确"
fi
