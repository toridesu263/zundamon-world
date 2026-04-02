using System;
using System.Text;
using System.Threading;
using System.IO;
using System.Diagnostics;

public class CPHInline
{
	public bool Execute()
	{
		// your main code goes here
		CPH.TryGetArg("messageStripped", out string message);
        long unixtime = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();
		//「とりです」の部分は自分のユーザー名に書き換えてください
        string path = "C:/Users/とりです/Desktop/zundamon-world/tts_queue_Rust";
		string pathOfText = $"C:/Users/とりです/Desktop/zundamon-world/tts_queue_Rust/twitch_{unixtime}_10.txt";
        File.WriteAllText(pathOfText, message);

		return true;
	}
}