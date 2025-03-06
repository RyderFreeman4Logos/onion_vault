use std::fmt::Write;

// Define a trait called `JsonCompactData` with one method to compact the 'data' field in JSON strings.
pub trait JsonCompactData {
    fn compact_data_field(&self) -> String;
}

// Implement the `JsonCompactData` trait for the `String` type.
impl JsonCompactData for String {
    fn compact_data_field(&self) -> String {
        // Initialize a new string to store the modified JSON.
        let mut result = String::new();
        // Flag to indicate if we're currently processing the 'data' field.
        let mut is_in_data = false;
        // Temporary string to build up the content of the 'data' field.
        let mut data_content = String::new();
        // Variable to keep track of current indentation.
        let mut current_indentation = String::new();

        // Iterate through each line of the JSON string.
        for line in self.lines() {
            // Update the current indentation based on the line containing the start of the 'data' array.
            if line.contains("\"data\": [") && current_indentation.is_empty() {
                current_indentation = line.chars().take_while(|&c| c != '"').collect();
            }

            // Check if this line starts the 'data' array.
            if line.contains("\"data\": [") {
                is_in_data = true;
                // Start building the data content with an opening bracket.
                data_content.push('[');
            } 
            // If we're in the data section and we've reached the end of the array.
            else if is_in_data && line.contains(']') {
                // Collect everything before the closing bracket, including any last comma if present.
                data_content.push_str(line.split(']').next().unwrap_or("").trim_start_matches(','));
                // Add the closing bracket without indentation.
                data_content.push(']');
                // Write the compacted 'data' field into the result with correct indentation:
                write!(result, "{}\"data\": {}", current_indentation, data_content).unwrap();
                result.push('\n');
                // Reset the flag and clear the temporary content.
                is_in_data = false;
                data_content.clear();
            } 
            // If we're still processing data but haven't reached the end.
            else if is_in_data {
                // Remove any leading or trailing spaces from the line but keep commas.
                let line = line.trim_matches(' ');
                // Append this line to our data content, keeping commas between elements.
                data_content.push_str(line);
            } 
            // For all other lines, just append them to the result as they are.
            else {
                result.push_str(line);
                result.push('\n');
            }
        }

        // Check if there's an extra newline at the end and remove it if present.
        if let Some('\n') = result.chars().last() {
            result.pop();
        }

        // Return the modified JSON string.
        result
    }
}
