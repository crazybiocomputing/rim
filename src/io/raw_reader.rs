use std::fs::File;
use std::io::Read;

/// Return a vector u8 containing the raw data of the image
///
/// # arguments
///
/// * `filename` - String containing the name of the raw file
///
fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn save_raw_file(name: &String){ // A voir si ca tourne bien en fonction
    let filename = format!("{}.raw", name);
    let file = File::create(filename);
    file.expect("REASON").write_all(&cc.buffer());
}

// TODO
/* 
    essayer de read file en vecteur de float
    read image depuis url (API ImageJ)
    save sous format TIF


/*
    /** Opens the image at 'filePath' using the format specified by 'fi'. */
    public static ImagePlus open(String filePath, FileInfo fi) {
        File f = new File(filePath);
        String parent = f.getParent();
        if (parent!=null)
            fi.directory = parent+ "/";
        fi.fileName = f.getName();
        return (new FileOpener(fi)).open(false);
    }   


    /** Opens all the images in the specified directory as a stack,
        using the format specified by 'fi'. */
    public static ImagePlus openAll(String directory, FileInfo fi) {
        ImagePlus imp = openAllVirtual(directory,fi);
        if (imp!=null)
            return imp.duplicate();
        else
            return null;
    }   

    /** Opens all the images in the specified directory as a virtual stack,
        using the format specified by 'fi'. */
    public static ImagePlus openAllVirtual(String directory, FileInfo fi) {
        String[] list = new File(directory).list();
        if (list==null)
            return null;
        FolderOpener fo = new FolderOpener();
        list = fo.trimFileList(list);
        list = fo.sortFileList(list);
        if (list==null)
            return null;
        directory = IJ.addSeparator(directory);
        FileInfo[] info = new FileInfo[list.length];
        for (int i=0; i<list.length; i++) {
            info[i] = (FileInfo)fi.clone();
            info[i].directory = directory;
            info[i].fileName = list[i];
        }
        VirtualStack stack = new FileInfoVirtualStack(info);
        ImagePlus imp = new ImagePlus(directory, stack);
        return imp;
    }   
    
}
*/